export function openModal(id: string) {
  const modal = document.getElementById(id);
  if (modal)
    (modal as any).showModal();
  else
    console.error("Modal could not be opened");
}
