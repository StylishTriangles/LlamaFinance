<script lang="ts" setup>
import { modalsID } from "~/config";
import { emitter } from "~/main";
import type { BasicAsset, BorrowData, WithdrawData } from "~/types";

const initialModalsData = {
  assetToDeposit: null as BasicAsset | null,

  assetToWithdraw: null as BasicAsset | null,
  availableToWithdraw: null as number | null,
  totalDeposited: null as number | null,
  aprPct: null as string | null,

  assetToBorrow: null as BasicAsset | null,
  availableToBorrow: null as number | null,

  assetToRepay: null as BasicAsset | null,

  assetToCollateralize: null as BasicAsset | null,

  assetToReduceCollateral: null as BasicAsset | null,

  txnHash: null as string | null,
};

const state = reactive({ ...initialModalsData });

onMounted(() => {
  emitter.on("open-deposit-modal", openDepositModal);
  emitter.on("open-withdraw-modal", openWithdrawModal);
  emitter.on("open-borrow-modal", openBorrowModal);
  emitter.on("open-repay-modal", openRepayModal);
  emitter.on("open-collateral-modal", openCollateralModal);
  emitter.on("open-reduce-col-modal", openReduceCollateralModal);
  emitter.on("txn-success", openTxnSuccessModal);
});

function openDepositModal(asset: BasicAsset) {
  closeModals();
  state.assetToDeposit = asset;
  openModal(modalsID.DEPOSIT);
}
function openWithdrawModal(data: WithdrawData) {
  closeModals();
  state.assetToWithdraw = data.asset;
  state.availableToWithdraw = data.available;
  state.totalDeposited = data.totalDeposited;
  state.aprPct = data.aprPct;
  openModal(modalsID.WITHDRAW);
}
function openBorrowModal(data: BorrowData) {
  closeModals();
  state.assetToBorrow = data.asset;
  state.availableToBorrow = data.available;
  openModal(modalsID.BORROW);
}
function openRepayModal(asset: BasicAsset) {
  closeModals();
  state.assetToRepay = asset;
  openModal(modalsID.REPAY);
}
function openCollateralModal(asset: BasicAsset) {
  closeModals();
  state.assetToCollateralize = asset;
  openModal(modalsID.COLLATERAL);
}
function openReduceCollateralModal(asset: BasicAsset) {
  closeModals();
  state.assetToReduceCollateral = asset;
  openModal(modalsID.REDUCE_COL);
}
function openTxnSuccessModal(hash: string) {
  closeModals();
  state.txnHash = hash;
  openModal(modalsID.TXN_SUCCESS);
}
async function openModal(id: string) {
  await nextTick(); // wait for modal that is going to be rendered
  const modal = document.getElementById(id);
  if (modal)
    (modal as any).showModal();
  else
    console.error("Modal could not be opened");
}
function closeModals() {
  Object.assign(state, { ...initialModalsData });
}
</script>

<template>
  <div>
    <BorrowModal
      v-if="state.assetToBorrow"
      :key="state.assetToBorrow.denom"
      :asset="state.assetToBorrow"
      :available="state.availableToBorrow!"
    />
    <RepayModal
      v-if="state.assetToRepay"
      :key="state.assetToRepay.denom"
      :asset="state.assetToRepay"
    />
    <CollateralModal
      v-if="state.assetToCollateralize"
      :key="state.assetToCollateralize.denom"
      :asset="state.assetToCollateralize"
    />
    <ReduceCollateralModal
      v-if="state.assetToReduceCollateral"
      :key="state.assetToReduceCollateral.denom"
      :asset="state.assetToReduceCollateral"
    />
    <DepositModal
      v-if="state.assetToDeposit"
      :key="state.assetToDeposit.denom"
      :asset="state.assetToDeposit"
    />
    <WithdrawModal
      v-if="state.assetToWithdraw"
      :key="state.assetToWithdraw.denom"
      :asset="state.assetToWithdraw"
      :available="state.availableToWithdraw!"
      :deposited="state.totalDeposited!"
      :apr-pct="state.aprPct!"
    />
    <TxnSuccessModal
      v-if="state.txnHash"
      :key="state.txnHash"
      :txn-hash="state.txnHash"
    />
  </div>
</template>
