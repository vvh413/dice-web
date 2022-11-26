import { Component, For, Show } from "solid-js";
import { IDice } from "./IDice.interface";
import styles from './Dice.module.css';

export const Dice: Component<IDice> = (dice) => {
  return (
    <div class={styles.dice}>
      <div class={styles.values}>
        <Show
          when={dice.values.length}
          fallback={<span class={styles.value}></span>}
        >
          <For each={dice.values}>{(num, i) => <span class={styles.value} data-index={i()}>{num}</span>}</For>
        </Show>
      </div>
      <div class={styles.stats}>
        <div class={styles.total}>total: {dice.total}</div>
        <div class={styles.min}>min: {dice.min}</div>
        <div class={styles.max}>max: {dice.max}</div>
      </div>
    </div>
  );
}
