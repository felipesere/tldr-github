<script>
  import GlowBox from '../GlowBox.svelte';
  import Indicator from './Indicator.svelte';
  export let items

  $: toBeDisplayed = items.slice(0, 3);
</script>


{#each toBeDisplayed as item}
  <div class="horizontal-flex">
    <Indicator /><GlowBox content={item} />
  </div>
{/each}

<style>
  .horizontal-flex {
    display: flex;
    flex-direction: row;
  }
</style>
