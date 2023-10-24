const MICRO_BASE = 10 ** 6;

export function convertMicroDenomToDenom(amount: number | string) {
  if (typeof amount === "string") {
    amount = Number(amount);
  }
  amount = amount / MICRO_BASE;
  return Number.isNaN(amount) ? 0 : amount;
}

export function convertDenomToMicroDenom(amount: number | string): string {
  if (typeof amount === "string") {
    amount = Number(amount);
  }
  amount = amount * MICRO_BASE;
  return Number.isNaN(amount) ? "0" : String(amount);
}

export function convertFromMicroDenom(denom: string) {
  return denom?.substring(1).toUpperCase();
}
