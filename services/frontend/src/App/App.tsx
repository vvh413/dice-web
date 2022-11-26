import axios from 'axios';
import { Component, createEffect, createSignal } from 'solid-js';
import { Dice } from '../Dice/Dice';
import { IDice } from '../Dice/IDice.interface';
import { Seed } from '../Seed/Seed';
import Settings from '../Settings';
import { ValueInput } from '../ValueInput/ValueInput';

import styles from './App.module.css';


const App: Component = () => {
  const [dice, setDice] = createSignal<IDice>({
    values: [],
    total: 0,
    min: 0,
    max: 0,
  });
  const [x, setX] = createSignal<number>(1);
  const [y, setY] = createSignal<number>(100);
  const [seed, setSeed] = createSignal<string>("");
  const [useSeed, setUseSeed] = createSignal<boolean>(false);

  const roll = async () => {
    try {
      const url = `${Settings.dice_api_url}/${x()}d${y()}` + (useSeed() ? `:${seed()}` : "")
      const res = await axios.get(url);
      setDice(res.data);
      const urlParts = res.request?.responseURL.split(":");
      setSeed(urlParts[urlParts.length - 1]);
    } catch (e) {
      console.log(axios.isAxiosError(e) ? e.message : e);
    }
  }

  createEffect(() => {
    roll()
  })

  return (
    <div class={styles.App}>
      <Dice {...dice()} />
      <Seed value={seed} setValue={setSeed} use={useSeed} setUse={setUseSeed} />
      <div class={styles.input}>
        <ValueInput value={x} setValue={setX} min={1} max={1000} />
        <div class={styles.d}>d</div>
        <ValueInput value={y} setValue={setY} min={1} max={1000} />
      </div>
      <button class={styles.roll} onClick={roll}>Roll</button>
    </div>
  );
};

export default App;
