import { ref, watch } from 'vue'
import { open } from '@tauri-apps/api/dialog'
import { GlobalSettings } from '../App.vue'

export type EmitType = {
  (e: 'updateGlobalSettings', settings: Partial<GlobalSettings>): void
}

export type PropsType = {
  globalSettings: GlobalSettings
}

export const setup = (props: PropsType, emits: EmitType) => {
  const rangeBegin = ref<number>(props.globalSettings.rangeBegin)
  const rangeEnd = ref<number>(props.globalSettings.rangeEnd)
  const outDir = ref<string>(props.globalSettings.outDir)

  watch(() => props.globalSettings, () => {
    rangeBegin.value = props.globalSettings.rangeBegin
    rangeEnd.value = props.globalSettings.rangeEnd
    outDir.value = props.globalSettings.outDir
  }, {deep: true})

  const updateGlobalSettings = (settings: {rangeBegin?: number, rangeEnd?: number, outDir?: string}) => {
    emits('updateGlobalSettings', settings)
  }

  const updateRangeBegin = (e: Event) => {
    const value = (e?.target as HTMLInputElement)?.value
    if(value != null) {
      updateGlobalSettings({rangeBegin: Number(value)})
    }
  }

  const updateRangeEnd = (e: Event) => {
    const value = (e?.target as HTMLInputElement)?.value
    if(value != null) {
      updateGlobalSettings({rangeEnd: Number(value)})
    }
  }

  const openDialog = async () => {
    let value = await open({
      directory: true,
    })

    if (value === null) {
      return
    } else if (Array.isArray(value)) {
      value = value[0]
    }

    updateGlobalSettings({outDir: value})
  }

  return {
    rangeBegin,
    rangeEnd,
    outDir,
    updateRangeBegin,
    updateRangeEnd,
    openDialog,
  }
}
