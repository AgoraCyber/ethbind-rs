struct Test(Client);
impl Test {
    pub fn deploy<C, P0, Ops>(
        context: C,
        weth: P0,
        ops: Ops,
    ) -> Result<TransactionReceipt, anyhow::Error>
    where
        C: TryInto<Client>,
        C::Error: Into<anyhow::Error>,
        Ops: TryInto<ClientOps>,
        Ops::Error: Into<anyhow::Error>,
        P0: TryInto<ethers_rs::Address>,
        P0::Error: Into<anyhow::Error>,
    {
        let context = context.try_into()?;
        let weth = weth.try_into()?;
        let ops = ops.try_into()?;
        let inputs = vec ! [weth . to_encodable () ;];
        let call_data = encode_inputs(inputs);
        context.deploy_contract(call_data, ops)
    }
}
