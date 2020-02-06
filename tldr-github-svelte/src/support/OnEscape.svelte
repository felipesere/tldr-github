<script>
  export let action = () => {};

    const keydown = ({keyCode}) => {
        // ESC key
        if (keyCode === 27) {
            action();
        }
    }
</script>

<svelte:window on:keydown={keydown}/>

