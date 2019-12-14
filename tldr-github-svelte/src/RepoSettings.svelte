<script>
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  export let repoId;
  let currentlyDeletingRepo;

  function doDelete(path) {
    fetch(`/api${path}`, { "method": "DELETE" })
  };

  function handleClick() {
    (currentlyDeletingRepo = async () => {
      await doDelete(`/repos/${repoId}`)
      setTimeout(() => {
        dispatch('repo-deleted');
      }, 500)
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
