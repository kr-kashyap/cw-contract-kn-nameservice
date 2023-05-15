use cosmwasm_std::{
    entry_point, to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, WasmMsg
};

use crate::coin_helpers::assert_sent_sufficient_coin;
use crate::error::ContractError;
use crate::msg::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse,
};
use crate::state::{Config, NameRecord, CONFIG, NAME_RESOLVER};
use cw20::{Cw20ExecuteMsg};

const MIN_NAME_LENGTH: u64 = 3;
const MAX_NAME_LENGTH: u64 = 64;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let config = Config {
        purchase_price: msg.purchase_price,
        transfer_price: msg.transfer_price,
        cw20_contract: msg.cw20_contract,
    };

    // TODO: Add a check for contract address validation.
    // let _rcpt_addr = deps.api.addr_validate(&config.cw20_contract)?;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { name, coin } => {
            execute_register(deps, env, info, name, coin)
        }
        ExecuteMsg::Transfer { name, to, coin } => {
            execute_transfer(deps, env, info, name, to, coin)
        }
    }
}

pub fn execute_register(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    coin: Coin,
    ) -> Result<Response, ContractError> {
    // we only need to check here - at point of registration
    let name_owner = info.clone().sender.into_string();
    validate_name(&name)?;
    let config = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(coin.clone(), config.purchase_price)?;

    let key = name.as_bytes();
    let record = NameRecord { owner: name_owner };

    if (NAME_RESOLVER.may_load(deps.storage, key)?).is_some() {
        // name is already taken
        return Err(ContractError::NameTaken { name });
    }

    // name is available
    NAME_RESOLVER.save(deps.storage, key, &record)?;

    // Create a CW20Msg of TransferFrom type.
    let msg = Cw20ExecuteMsg::TransferFrom{
        owner:info.clone().sender.into_string(),
        recipient: env.clone().contract.address.into_string(), 
        amount: coin.amount, 
    };

    // Add a callback of above message 
    Ok(Response::new()
        .add_message(
        WasmMsg::Execute {
                contract_addr : config.cw20_contract,
                msg: to_binary(&msg)?,
                funds: vec![],
            }
        )
    )
}

pub fn execute_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    to: String,
    coin: Coin,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(coin.clone(), config.transfer_price)?;

    let new_owner = deps.api.addr_validate(&to)?;
    let key = name.as_bytes();
    NAME_RESOLVER.update(deps.storage, key, |record| {
        if let Some(mut record) = record {
            if info.sender != record.owner {
                return Err(ContractError::Unauthorized {});
            }

            record.owner = new_owner.clone().into_string();
            Ok(record)
        } else {
            Err(ContractError::NameNotExists { name: name.clone() })
        }
    })?;

    // Create a CW20Msg of TransferFrom type.
    let msg = Cw20ExecuteMsg::TransferFrom{
        owner:info.clone().sender.into_string(),
        recipient: env.clone().contract.address.into_string(), 
        amount: coin.amount, 
    };

    // Add a callback of above message 
    Ok(Response::new()
        .add_message(
        WasmMsg::Execute {
                contract_addr : config.cw20_contract,
                msg: to_binary(&msg)?,
                funds: vec![],
            }
        )
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ResolveRecord { name } => query_resolver(deps, env, name),
        QueryMsg::Config {} => to_binary::<ConfigResponse>(&CONFIG.load(deps.storage)?.into()),
    }
}

fn query_resolver(deps: Deps, _env: Env, name: String) -> StdResult<Binary> {
    let key = name.as_bytes();

    let address = match NAME_RESOLVER.may_load(deps.storage, key)? {
        Some(record) => Some(String::from(&record.owner)),
        None => None,
    };
    let resp = ResolveRecordResponse { address };

    to_binary(&resp)
}

// let's not import a regexp library and just do these checks by hand
fn invalid_char(c: char) -> bool {
    let is_valid =
        c.is_ascii_digit() || c.is_ascii_lowercase() || (c == '.' || c == '-' || c == '_');
    !is_valid
}

/// validate_name returns an error if the name is invalid
/// (we require 3-64 lowercase ascii letters, numbers, or . - _)
fn validate_name(name: &str) -> Result<(), ContractError> {
    let length = name.len() as u64;
    if (name.len() as u64) < MIN_NAME_LENGTH {
        Err(ContractError::NameTooShort {
            length,
            min_length: MIN_NAME_LENGTH,
        })
    } else if (name.len() as u64) > MAX_NAME_LENGTH {
        Err(ContractError::NameTooLong {
            length,
            max_length: MAX_NAME_LENGTH,
        })
    } else {
        match name.find(invalid_char) {
            None => Ok(()),
            Some(bytepos_invalid_char_start) => {
                let c = name[bytepos_invalid_char_start..].chars().next().unwrap();
                Err(ContractError::InvalidCharacter { c })
            }
        }
    }
}
