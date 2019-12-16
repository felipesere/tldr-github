<script>
  import { fade } from 'svelte/transition';
  import { onMount } from 'svelte';

  let error = false;

  onMount(() => {
    setTimeout(() => {
      error = "Something went wrong!"
    }, 3000)
  })

  let dismiss = () => {
    error = false
  }

</script>

<div class="toast-container">
  {#if error }
  <article transition:fade="{{duration: 500}}" class="message is-warning">
    <div class="message-header">
      <p>Warning</p>
      <button class="delete" aria-label="delete" on:click={dismiss}></button>
    </div>
    <div class="message-body">
      {error}
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
