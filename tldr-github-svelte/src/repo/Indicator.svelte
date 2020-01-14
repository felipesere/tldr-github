<script context="module">
  export const Recency = {
    "HIGH": 'high',
    "MEDIUM": 'medium',
    "LOW": 'low',
    "NONE": 'none',
  };
</script>
<script>
  let random = () => {
    let possibleValues = Object.values(Recency);
    return possibleValues[Math.floor(Math.random() * possibleValues.length)];
  };

  let activity = random();
  let clazz = `dot ${activity}`
</script>

<span class={clazz}></span>


<style>
  .dot {
    height: 10px;
    width: 10px;
    border-radius: 50%;
    display: inline-block;
    margin-right: 10px;

    align-self: center;
    flex-shrink: 0;
  }

  .high {
    background-color: #3182ce;
  }

  .medium {
    background-color: #90cdf4
  }

  .low {
    background-color: #cbd5e0
  }

  .none {
    visibility: hidden;
  }
</style>
