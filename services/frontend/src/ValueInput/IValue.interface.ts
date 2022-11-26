import { Accessor, Setter } from "solid-js"

export interface IValue {
  value: Accessor<number>
  setValue: Setter<number>
  min: number
  max: number
}
