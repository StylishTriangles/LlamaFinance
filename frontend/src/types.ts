import type { EventType } from "mitt";

export interface TableColumn {
  header: string;
  accessor: string;
}

export interface BasicAsset {
  denom: string;
  name: string;
  icon: string;
  decimals: number;
  balance: number;
  balanceUSD: number;
  price: number;
  precision: number;
}

export interface EmitterEvents extends Record<EventType, any> {
  "txn-success": string;
  "open-deposit-modal": BasicAsset;
  "open-withdraw-modal": BasicAsset;
  "open-borrow-modal": BasicAsset;
  "open-repay-modal": any;
  "open-collateral-modal": BasicAsset;
  "open-reduce-col-modal": any;
}
