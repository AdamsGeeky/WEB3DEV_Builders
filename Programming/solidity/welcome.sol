// SPDX-License-Identifier: MIT
pragma solidity >=0.7.0 <0.9.0;

contract Welcome {
    constructor() {
        
    }

    function Hello() public  pure  returns (string memory){
        return 'Welcome unbreakerble Promise writer';
    }

    function getResult() public pure returns (uint) {
        uint a = 1;
        uint b = 2;
        uint result = a + b;
        return result;
    }
}
