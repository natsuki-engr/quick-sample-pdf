export interface EmitType {
  (e: 'click-remove-btn'): void
}

export const setup = (emit: EmitType) => {
  const clickRemoveBtn = () => {
    emit('click-remove-btn')
  }

  return {
    clickRemoveBtn,
  }
}
