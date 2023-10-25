<script lang="ts" setup>
import { modalsID } from "~/config";
import { emitter } from "~/main";

const initialModalsData = {
  assetToDeposit: null as any,
  assetToWithdraw: null as any,
  assetToBorrow: null as any,
  assetToRepay: null as any,
  assetToCollateralize: null as any,
  assetToReduceCollateral: null as any,
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

function openDepositModal(asset: any) {
  closeModals();
  state.assetToDeposit = asset;
  openModal(modalsID.DEPOSIT);
}
function openWithdrawModal(asset: any) {
  closeModals();
  state.assetToWithdraw = asset;
  openModal(modalsID.WITHDRAW);
}
function openBorrowModal(asset: any) {
  closeModals();
  state.assetToBorrow = asset;
  openModal(modalsID.BORROW);
}
function openRepayModal(asset: any) {
  closeModals();
  state.assetToRepay = asset;
  openModal(modalsID.REPAY);
}
function openCollateralModal(asset: any) {
  closeModals();
  state.assetToCollateralize = asset;
  openModal(modalsID.COLLATERAL);
}
function openReduceCollateralModal(asset: any) {
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
      :key="state.assetToBorrow"
      :asset="state.assetToBorrow"
    />
    <RepayModal
      v-if="state.assetToRepay"
      :key="state.assetToRepay"
      :asset="state.assetToRepay"
    />
    <CollateralModal
      v-if="state.assetToCollateralize"
      :key="state.assetToCollateralize"
      :asset="state.assetToCollateralize"
    />
    <ReduceCollateralModal
      v-if="state.assetToReduceCollateral"
      :key="state.assetToReduceCollateral"
      :asset="state.assetToReduceCollateral"
    />
    <DepositModal
      v-if="state.assetToDeposit"
      :key="state.assetToDeposit"
      :asset="state.assetToDeposit"
    />
    <WithdrawModal
      v-if="state.assetToWithdraw"
      :key="state.assetToWithdraw"
      :asset="state.assetToWithdraw"
    />
    <TxnSuccessModal
      v-if="state.txnHash"
      :key="state.txnHash"
      :txn-hash="state.txnHash"
    />
  </div>
</template>
