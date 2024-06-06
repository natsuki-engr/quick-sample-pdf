export interface EmitType {
  (e: 'click-start-btn'): void
}

export const setup = (emits: EmitType) => {
  const onClickStartBtn = () => {
    console.log('onClickStartBtn')
    emits('click-start-btn')
  }

  return {
    onClickStartBtn,
  }
}
