import { Component } from "solid-js";
import styles from './ValueInput.module.css';
import { IValue } from "./IValue.interface";

export const ValueInput: Component<IValue> = ({ value, setValue, min, max }) => {
  const handleDecrease = (_: Event) => {
    if (value() > min)
      setValue(value() - 1)
  }
  const handleIncrease = (_: Event) => {
    if (value() < max)
      setValue(value() + 1)
  }
  const handleInput = (event: InputEvent) => {
    const value = event.currentTarget?.value;
    if (!/^\d+$/.test(value))
      return
    setValue(parseInt(value) || min)
  }
  return (
    <div class={styles.valueInput}>
      <button onClick={handleIncrease}>+</button>
      <input onInput={handleInput} value={value()} />
      <button onClick={handleDecrease}>-</button>
    </div>
  )
}
