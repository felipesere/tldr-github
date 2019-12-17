import { writable } from 'svelte/store';

export const error = writable(false);

export const newError = (msg) => {
  error.set({msg})
}

export const clear = () => {
  error.set(false)
}
