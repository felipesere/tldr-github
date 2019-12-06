<script>
  import GlowBox from './GlowBox.svelte';
  export let prs;
</script>


<div>
  {prs.length} new Pull Requests:
  {#each prs as pr}
    <GlowBox content={pr} />
  {/each}
</div>
