import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Button from '@mui/material/Button';

function App() {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");

  async function createTask() {
    const result = await invoke<string>("create_task", {name, description});
    console.log(result);
  }

  async function listTasks() {
    const result = await invoke<string>("list_tasks");
    console.log(result);
  }

  return (
    <main className="container">
      <h1>Create a task</h1>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          createTask();
        }}
      >
        <input
          id="name-input"
          value={name}
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <input
          id="description-input"
          value={description}
          onChange={(e) => setDescription(e.currentTarget.value)}
          placeholder="Enter a description..."
        />
        <Button type="submit">Create</Button>
        <Button
          variant="contained"
          onClick={async () => {
            await listTasks();
          }}>
          List
      </Button>
      </form>


    </main>
  );
}

export default App;
