<script>
import { user } from "./store.js";
export let name;
</script>

<main>
  <div id="main">
    <h1><span style="font-weight:bold">Hello</span> {name}!</h1>
    <p>Welcome to my demo webapp. Here's a short blurb that sounds neat.</p>
    <form class="email-asker">
      <input type=text bind:value={$user.email} placeholder="Enter your email" />
    </form>
  </div>
  <div id="key-features">
    some key features:
  </div>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
