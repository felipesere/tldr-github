<script>
  import {clear, error} from './errorStore.js';
  import { fade } from 'svelte/transition';
  import { onMount } from 'svelte';

  let message = false
  error.subscribe(value => {
    message = value.msg
  })
</script>

<div class="toast-container">
  {#if message }
  <article transition:fade="{{duration: 500}}" class="message is-warning">
    <div class="message-header">
      <p>Warning</p>
      <button class="delete" aria-label="delete" on:click={clear}></button>
    </div>
    <div class="message-body">
      {message}
    </div>
  </article>
  {/if}
</div>

<style>
.toast-container {
  position: absolute;
  top: 0;
  right: 0;
  z-index: 100;
}
</style>
