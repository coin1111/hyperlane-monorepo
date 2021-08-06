/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */

import { Signer, utils, Contract, ContractFactory, Overrides } from "ethers";
import { Provider, TransactionRequest } from "@ethersproject/providers";
import type {
  TestXappConnectionManager,
  TestXappConnectionManagerInterface,
} from "../TestXappConnectionManager";

const _abi = [
  {
    inputs: [],
    name: "localDomain",
    outputs: [
      {
        internalType: "uint32",
        name: "",
        type: "uint32",
      },
    ],
    stateMutability: "pure",
    type: "function",
  },
];

const _bytecode =
  "0x6080604052348015600f57600080fd5b5060878061001e6000396000f3fe6080604052348015600f57600080fd5b506004361060285760003560e01c80638d3638f414602d575b600080fd5b6033604c565b6040805163ffffffff9092168252519081900360200190f35b60059056fea2646970667358221220a76d02d44e887b7ef067ed0a946960a8cd9f75faf14cb24e5371540a3317bb8964736f6c63430007060033";

export class TestXappConnectionManager__factory extends ContractFactory {
  constructor(signer?: Signer) {
    super(_abi, _bytecode, signer);
  }

  deploy(
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<TestXappConnectionManager> {
    return super.deploy(overrides || {}) as Promise<TestXappConnectionManager>;
  }
  getDeployTransaction(
    overrides?: Overrides & { from?: string | Promise<string> }
  ): TransactionRequest {
    return super.getDeployTransaction(overrides || {});
  }
  attach(address: string): TestXappConnectionManager {
    return super.attach(address) as TestXappConnectionManager;
  }
  connect(signer: Signer): TestXappConnectionManager__factory {
    return super.connect(signer) as TestXappConnectionManager__factory;
  }
  static readonly bytecode = _bytecode;
  static readonly abi = _abi;
  static createInterface(): TestXappConnectionManagerInterface {
    return new utils.Interface(_abi) as TestXappConnectionManagerInterface;
  }
  static connect(
    address: string,
    signerOrProvider: Signer | Provider
  ): TestXappConnectionManager {
    return new Contract(
      address,
      _abi,
      signerOrProvider
    ) as TestXappConnectionManager;
  }
}
