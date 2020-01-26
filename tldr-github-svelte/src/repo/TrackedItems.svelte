<script>
  import GlowBox from '../atoms/GlowBox.svelte';
  import Indicator from '../atoms/Indicator.svelte';
  export let items;

  $: toBeDisplayed = items.slice(0, 3);
</script>

<ul>
    {#each toBeDisplayed as item}
        <li class="horizontal-flex">
            <Indicator time={item.last_updated} /><GlowBox content={item} />
        <li>
    {/each}
</ul>

<style>
  ul {
    list-style-type: none;
    margin-left: 0;
  }
</style>
