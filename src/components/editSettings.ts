import { ref } from 'vue'
import { open } from '@tauri-apps/api/dialog'

export type EmitType = {
  (e: 'updateOutDir', value: string): void
}

export const setup = (emits: EmitType) => {
  const rangeBegin = ref<number>(0)
  const rangeEnd = ref<number>(3)

  const openDialog = async () => {
    let value = await open({
      directory: true,
    })

    if (value === null) {
      return
    } else if (Array.isArray(value)) {
      value = value[0]
    }

    emits('updateOutDir', value)
  }

  return {
    rangeBegin,
    rangeEnd,
    openDialog,
  }
}
