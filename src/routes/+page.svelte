<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import "../app.css";
  import Button from "@/components/ui/button/button.svelte";
  import Input from "@/components/ui/input/input.svelte";
  import { Trash } from "lucide-svelte";
  import Database from "@tauri-apps/plugin-sql";

  type User = {
    id: number;
    name: string;
    email: string;
  };

  let name = "";
  let email = "";
  let isLoadingUsers = true;
  let users: User[] = [];
  let errorMessage = "";

  async function loadUsers() {
    try {
      const db = await Database.load("sqlite:users.db");

      // Ensure table exists
      await db.execute(`
        CREATE TABLE IF NOT EXISTS users (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          name TEXT NOT NULL,
          email TEXT
        )
      `);

      const dbUsers = await db.select<User[]>("SELECT * FROM users");

      errorMessage = "";
      users = dbUsers;
      isLoadingUsers = false;
    } catch (error) {
      console.error(error);
      errorMessage = "Failed to load users check console for details";
    }
  }

  async function deleteUser(id: number) {
    try {
      const db = await Database.load("sqlite:users.db");
      await db.execute("DELETE FROM users WHERE id = $1", [id]);
      await loadUsers();
    } catch (error) {
      console.error(error);
      errorMessage = "Failed to delete user";
    }
  }

  // async function setUser(user: User) {
  //   try {
  //     isLoadingUsers = true;
  //     const db = await Database.load("sqlite:users.db");
  //     await db.execute("INSERT INTO users (name, email) VALUES ($1, $2)", [
  //       name,
  //       user.email,
  //     ]);

  //     await loadUsers();
  //   } catch (error) {
  //     console.error(error);
  //     errorMessage = "Failed to add user check console for details";
  //   }
  // }

  // Initial load
  loadUsers();
</script>

<main class="container mx-auto p-4 max-w-2xl">
  <div id="form" class="fill-slate-300">
    <h1 class="text-2xl font-bold mb-6 text-left">User Management</h1>

    <!-- Add User Form -->
    <form
      class="mb-8 flex gap-4"
      on:submit={async (e) => {
        e.preventDefault();
        try {
          isLoadingUsers = true;
          const db = await Database.load("sqlite:users.db");
          await db.execute("INSERT INTO users (name, email) VALUES ($1, $2)", [
            name,
            email,
          ]);
          await loadUsers();
        } catch (error) {
          console.error(error);
          errorMessage = "Failed to add user";
        }
      }}
    >
      <div>
        <Input id="name-input" placeholder="Enter name..." bind:value={name} />
      </div>
      <div>
        <Input
          id="email-input"
          type="email"
          placeholder="Enter email..."
          bind:value={email}
        />
      </div>
      <Button type="submit">Add User</Button>
    </form>
  </div>


  <!-- User List -->

  <div class="space-y-4">
    <h2 class="text-xl font-semibold text-left">Users</h2>
    {#if users.length === 0}
      <p class="text-gray-500">No users found</p>
    {:else}
      <div id="user-list-container" class="overflow-y-auto max-h-[500px]">
        {#each users as user (user.id)}
          <div class="py-3 px-4 bg-slate-100 rounded-md my-3 flex gap-4 justify-between" id="user-list-box">
            <div class="flex gap-4 items-center">
              <h3 class="font-medium text-slate-500 min-w-20 text-left">{user.name}</h3>
              <p class="text-slate-500">{user.email}</p>
            </div>
            <Button variant="outline" onclick={() => deleteUser(user.id)}>
              <Trash />
            </Button>

          </div>
        {/each}
      </div>
    {/if}
  </div>
  {#if errorMessage}
    <p class="text-red-500 mb-4">{errorMessage}</p>
  {/if}
</main>
