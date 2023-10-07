contract;

struct TestEvent{
    n: u64
}

abi MyContract {
    fn test_function();
}

impl MyContract for Contract {
    fn test_function() {
        log(TestEvent{n: 1});
        log(TestEvent{n: 2});
    }
}

/* 
Network: http://beta-4.fuel.network/graphql
Contract ID: fuel1pztzy59xunq2scsq9tkufxnqm35kpqnwxzwu70ejtq936yt5f06qvkettc
Contract ID: 0x08962250a6e4c0a862002aedc49a60dc6960826e309dcf3f32580b1d11744bf4
Deployed in block 0x5e2a917d2c7ee5d4ba082af58214d6b515177df916fec405be335ef19fdc90ba
 */