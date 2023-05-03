Steps for registering a nameservice using CW20 tokens


1. Store CW-tokens/Staking contract. [Here Unbonding period field is removed from instantiate]

orbitiumd tx wasm store ~/Downloads/Github/rust-practice/cw-tokens/target/wasm32-unknown-unknown/release/cw20_staking_copy.wasm --from alice --gas-prices 0.02uorb --gas auto --gas-adjustment 1.75 -y

2. Instantiate CW-tokens/Staking contract.

orbitiumd tx wasm instantiate 1 '{"decimals": 6 , "name" : "kr-kn" , "symbol" : "kr-kn" , "validator" : "orbvaloper1nj257hlyn4n7f7y0qr8qcu72ugdkhnkce7fvx0" , "exit_tax" : "0.01" , "min_withdrawal" : "100"}' --from alice --gas-prices 0.02uorb --gas auto --gas-adjustment 1.5 --label "Staking_1" --no-admin -y

3. Store KN-nameservice contract.

orbitiumd tx wasm store ~/Downloads/Github/cw-contract-kn-nameservice/target/wasm32-unknown-unknown/release/kn_nameservice.wasm --from alice --gas-prices 0.02uorb --gas auto --gas-adjustment 1.75 -y

4. Instantiate KN-nameservice contract.

orbitiumd tx wasm instantiate 2 '{"purchase_price":{"amount":"10","denom":"kr-kn"},"transfer_price":{"amount":"99","denom":"kr-kn"}}' --label "name_service" --no-admin --from alice --gas-prices 0.02uorb --gas auto --gas-adjustment 1.4 -y

5. Bond in CW-tokens/Staking contract for minting.

orbitiumd tx wasm execute orb14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s682k4w '{"bond":{}}' --amount 100000000uorb --from alice --gas-prices 0.02uorb --gas auto --gas-adjustment 1.5 -y

6. Query CW20 balance.

 orbitiumd q wasm contract-state smart orb14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s682k4w '{"balance":{"address":"orb1nj257hlyn4n7f7y0qr8qcu72ugdkhnkcvnf3wd"}}'

7. Register a name on nameservice contract using CW20 token.

orbitiumd tx wasm execute orb14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s682k4w '{"send" : { "contract" : "orb1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqkj05hg" , "amount" : "11" , "msg" : "eyJyZWdpc3RlciI6eyJuYW1lIjoiYWxpY2UiLCJjb2luIjp7ImRlbm9tIjoia3Ita24iLCJhbW91bnQiOiIxMSJ9fX0=" }}' --from alice --gas-prices 0.02uorb --gas auto --gas-adjustment 1.5 -y

8. Fetch owner of the name.

orbitiumd q wasm contract-state smart orb1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqkj05hg '{ "resolve_record" : { "name" : "alice" }}'

9. Fetch CW20 balance.

orbitiumd q wasm contract-state smart orb14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s682k4w '{"balance":{"address":"orb1nj257hlyn4n7f7y0qr8qcu72ugdkhnkcvnf3wd"}}'
