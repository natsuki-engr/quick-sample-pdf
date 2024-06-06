import { ref } from "vue"

export const setup = () => {
  const rangeBegin = ref<number>(0)
  const rangeEnd = ref<number>(3)

  return {
    rangeBegin,
    rangeEnd,
  }
}
