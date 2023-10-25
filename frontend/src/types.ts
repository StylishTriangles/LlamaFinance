import type { EventType } from "mitt";

export interface TableColumn {
  header: string;
  accessor: string;
}

export interface EmitterEvents extends Record<EventType, any> {
  "txn-success": string;
  "open-deposit-modal": any;
  "open-withdraw-modal": any;
  "open-borrow-modal": any;
  "open-collateral-modal": any;
}
