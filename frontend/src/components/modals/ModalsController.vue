<script lang="ts" setup>
import { modalsID } from "~/config";
import { emitter } from "~/main";

const initialModalsData = {
  assetToDeposit: null as any,
  assetToWithdraw: null as any,
  assetToBorrow: null as any,
  assetToCollateralize: null as any,
  txnSuccessData: null as any,
};

const state = reactive({ ...initialModalsData });

onMounted(() => {
  // emitter.on("txn-success", openTxnSuccessModal);
  emitter.on("open-deposit-modal", openDepositModal);
  emitter.on("open-withdraw-modal", openWithdrawModal);
  emitter.on("open-borrow-modal", openBorrowModal);
  emitter.on("open-collateral-modal", openCollateralModal);
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
function openCollateralModal(asset: any) {
  closeModals();
  state.assetToCollateralize = asset;
  openModal(modalsID.COLLATERAL);
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
    <CollateralModal
      v-if="state.assetToCollateralize"
      :key="state.assetToCollateralize"
      :asset="state.assetToCollateralize"
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
  </div>
</template>
