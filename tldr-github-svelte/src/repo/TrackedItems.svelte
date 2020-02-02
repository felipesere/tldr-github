<script>
  import GlowBox from '../atoms/GlowBox.svelte';
  import Indicator from '../atoms/Indicator.svelte';
  export let items;

  $: toBeDisplayed = items.slice(0, 3);
</script>

<ul class="stack-sm ml-0 list-none">
    {#each toBeDisplayed as item (item.nr)}
        <li class="flex">
            <Indicator time={item.last_updated} /><GlowBox content={item} />
        <li>
    {/each}
</ul>

