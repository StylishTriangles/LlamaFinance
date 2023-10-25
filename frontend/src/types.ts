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

export interface BorrowData {
  asset: BasicAsset;
  available: number;
}

export interface WithdrawData {
  asset: BasicAsset;
  available: number;
  totalDeposited: number;
  aprPct: string;
}

export interface EmitterEvents extends Record<EventType, any> {
  "txn-success": string;
  "open-deposit-modal": BasicAsset;
  "open-withdraw-modal": WithdrawData;
  "open-borrow-modal": BorrowData;
  "open-repay-modal": any;
  "open-collateral-modal": BasicAsset;
  "open-reduce-col-modal": any;
}
