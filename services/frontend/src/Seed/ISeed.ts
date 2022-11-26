import { Accessor, Setter } from "solid-js"

export interface ISeed {
  value: Accessor<string>,
  setValue: Setter<string>,
  use: Accessor<boolean>
  setUse: Setter<boolean>
}
