<script>
  import { createEventDispatcher } from 'svelte';
  import { newError } from '../errors/errorStore.js';

  const dispatch = createEventDispatcher();

  export let repoId;
  let currentlyDeletingRepo;

  function doDelete(path) {
    return fetch(`/api${path}`, { "method": "DELETE" })
  };

  function handleClick() {
    (currentlyDeletingRepo = async () => {
      let response = await doDelete(`/repos/${repoId}`)
      if (!response.ok) {
          let body = await response.json();
          console.log(body)
          newError(`Unable to delete repo: ${body.error}`)
          currentlyDeletingRepo = undefined
      } else {
        setTimeout(() => {
          dispatch('repo-deleted');
          currentlyDeletingRepo = undefined
        }, 500)
      }
    })()
  };

</script>

<section class="content">
  <div class="thing">
    <button class="button is-normal" class:is-loading={currentlyDeletingRepo} on:click|preventDefault={handleClick}>Delete</button>
    <p class="grow is-normal">to stop tracking this repo</p>
    <div>
</section>

<style>
.thing {
  display: flex;
  flex-direction: row;
}

p {
  margin-top: auto;
  margin-bottom: auto !important;
  padding-left: 5px;
}
</style>
