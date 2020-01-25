<script>
  import GlowBox from '../GlowBox.svelte';
  import Indicator from './Indicator.svelte';
  export let items;

  $: toBeDisplayed = items.slice(0, 3);
</script>

<ul>
    {#each toBeDisplayed as item}
        <li class="horizontal-flex">
            <Indicator /><GlowBox content={item} />
        <li>
    {/each}
</ul>

<style>
  ul {
    list-style-type: none;
    margin-left: 0;
  }
</style>
