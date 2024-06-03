import { open } from "@tauri-apps/api/dialog";

export interface EmitType {
  (e: 'select', value: string[]): void
}

export const setup = (emits: EmitType) => {
  const openDialog = async () => {
    let value = await open({
      filters: [{ name: 'PDF', extensions: ['pdf'] }],
    })
    if (value === null) {
      return
    } else if (!Array.isArray(value)) {
      value = [value]
    }

    emits('select', value)
  }


  return {
    openDialog,
  }
}
