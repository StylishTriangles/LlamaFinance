export function validateInput(
  value: string | number | null,
  maxValue: number,
  decimalPoints: number,
  assetName: string,
  allowZero = false,
): string {
  if ((!allowZero && Number(value) <= 0) || Number.isNaN(Number(value)))
    return "Invalid amount";

  if (value && Number(value) > maxValue)
    return "Insufficient funds";

  if (
    value
    && Number(value).toFixed(decimalPoints).length < value.toString().length
  )
    if (decimalPoints)
      return `${assetName} has ${decimalPoints} decimal points`;
    else return `${assetName} does not have decimal points`;

  return "";
}
