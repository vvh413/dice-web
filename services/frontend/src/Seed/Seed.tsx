import { Component } from "solid-js";
import { ISeed } from "./ISeed";
import styles from './Seed.module.css';


export const Seed: Component<ISeed> = ({ value, setValue, use, setUse }) => {
  const handleInputChange = (event: Event) => {
    setValue(event.currentTarget?.value)
  }
  const handleUseChange = (_: Event) => {
    setUse(!use())
  }

  return (
    <div class={styles.seed}>
      <input class={styles.value} type="text" onInput={handleInputChange} value={value()} />
      <label class={styles.use}>
        lock seed
        <input type="checkbox" checked={use()} onInput={handleUseChange} />
      </label>
    </div>
  )
}
