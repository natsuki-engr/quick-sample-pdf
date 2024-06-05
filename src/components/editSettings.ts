import { ref } from "vue"

export interface EmitType {
  (e: 'click-start-btn'): void
}

export const setup = (emits: EmitType) => {
  const rangeBegin = ref<number>(0)
  const rangeEnd = ref<number>(3)

  const onClickStartBtn = () => {
    console.log('onClickStartBtn')
    emits('click-start-btn')
  }

  return {
    rangeBegin,
    rangeEnd,
    onClickStartBtn,
  }
}
