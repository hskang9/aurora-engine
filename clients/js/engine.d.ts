import NEAR from 'near-api-js';
export { getAddress as parseAddress } from '@ethersproject/address';
export { arrayify as parseHexString } from '@ethersproject/bytes';
export declare class Engine {
    near: NEAR.Near;
    signer: NEAR.Account;
    contract: string;
    constructor(near: NEAR.Near, signer: NEAR.Account, contract: string);
    static connect(options: any, env: any): Promise<Engine>;
    initialize(options: any): Promise<any>;
    getVersion(): Promise<string>;
    getOwner(): Promise<string>;
    getBridgeProvider(): Promise<string>;
    getChainID(): Promise<bigint>;
    deployCode(bytecode: string | Uint8Array): Promise<string>;
    getCode(address: string): Promise<Uint8Array>;
    getBalance(address: string): Promise<bigint>;
    getNonce(address: string): Promise<bigint>;
    getStorageAt(address: string, key: string): Promise<Uint8Array>;
    callFunction(methodName: string, args?: Uint8Array | null): Promise<any>;
    callMutativeFunction(methodName: string, args?: Uint8Array | null): Promise<any>;
}
