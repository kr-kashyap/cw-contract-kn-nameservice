## Steps for registering a nameservice using CW20 tokens

1. Store CW-tokens/Staking contract. [Here Unbonding period field is removed from instantiate]

`orbitiumd tx wasm store ~/Downloads/Github/rust-practice/cw-tokens/target/wasm32-unknown-unknown/release/cw20_staking_copy.wasm --from alice --gas-prices 0.02uorb --gas auto --gas-adjustment 1.75 -y`

2. Instantiate CW-tokens/Staking contract.

`orbitiumd tx wasm instantiate 1 '{"decimals": 6 , "name" : "kr-kn" , "symbol" : "kr-kn" , "validator" : "orbvaloper1nj257hlyn4n7f7y0qr8qcu72ugdkhnkce7fvx0" , "exit_tax" : "0.01" , "min_withdrawal" : "100"}' --from alice --gas-prices 0.02uorb --gas auto --gas-adjustment 1.5 --label "Staking_1" --no-admin -y`

3. Bond in CW-Staking contract ( for minting).

`orbitiumd tx wasm execute orb14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s682k4w '{"bond":{}}' --amount 100000000uorb --from alice --gas-prices 0.02uorb --gas auto --gas-adjustment 1.5 -y`

4. Query CW20 balance.

 `orbitiumd q wasm contract-state smart orb14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s682k4w '{"balance":{"address":"orb1nj257hlyn4n7f7y0qr8qcu72ugdkhnkcvnf3wd"}}'`

5. Store KN-nameservice contract.

`orbitiumd tx wasm store ~/Downloads/Github/cw-contract-kn-nameservice/target/wasm32-unknown-unknown/release/kn_nameservice.wasm --from alice --gas-prices 0.02uorb --gas auto --gas-adjustment 1.75 -y`

6. Instantiate KN-nameservice contract. 

`orbitiumd tx wasm instantiate 5 '{"purchase_price":{"amount" : "10" , "denom" : "kr-kn" } , "transfer_price" : { "amount" : "99" , "denom" : "kr-kn" }, "cw20_contract" : "orb14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s682k4w" }' --label "name_service_5" --admin $(orbitiumd keys show alice -a) --from bob --gas-prices 0.02uorb --gas auto --gas-adjustment 1.4 -y`

7. Provide allowance to the nameservice contract from CW20 contract.

`orbitiumd tx wasm execute orb14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s682k4w '{"increase_allowance" : { "spender" : "orb1j08452mqwadp8xu25kn9rleyl2gufgfjnv0sn8dvynynakkjukcq6ucya5" , "amount" : "1000" , "expires" : null }}' --gas-prices 0.02uorb --gas auto --gas-adjustment 1.5 --from alice -y`

8. Register a name on nameservice contract using CW20 token.

`orbitiumd tx wasm execute orb1j08452mqwadp8xu25kn9rleyl2gufgfjnv0sn8dvynynakkjukcq6ucya5 '{"register" : {"name" : "abcd" , "coin" : {"amount" : "10", "denom" : "kr-kn"}}}' --from alice --gas-prices 0.02uorb --gas auto --gas-adjustment 1.5 -y`

9. Fetch owner of the name.

`orbitiumd q wasm contract-state smart orb1j08452mqwadp8xu25kn9rleyl2gufgfjnv0sn8dvynynakkjukcq6ucya5 '{ "resolve_record" : { "name" : "abcd" }}'`

10. Fetch CW20 balance.

`orbitiumd q wasm contract-state smart orb14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s682k4w '{"balance":{"address":"orb1nj257hlyn4n7f7y0qr8qcu72ugdkhnkcvnf3wd"}}'`
