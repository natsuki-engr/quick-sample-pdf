import { ref, watch } from 'vue'
import { open } from '@tauri-apps/api/dialog'

export type EmitType = {
  (e: 'updateOutDir', value: string): void
}

export type PropsType = {
  defaultRangeEnd: number
  outDir: string
}

export const setup = (props: PropsType, emits: EmitType) => {
  const rangeBegin = ref<number>(1)
  const rangeEnd = ref<number>(props.defaultRangeEnd)

  watch(() => props.defaultRangeEnd, () => {
    rangeEnd.value = props.defaultRangeEnd
  })

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
