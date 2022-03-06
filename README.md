# Linking holochain entries
Solution to the [Anchors](https://holochain-gym.github.io/developers/intermediate/anchors/) exercise on the Holochain Gym. 

`create_post` creates an entry and links it to a post anchor.  
`get_all_posts` searches all elements linked to the anchor defined above and returns a vector of entries (in this case posts).  

This was my first time not using the solutions provided at all, however I did refer back to previous work for unwrapping Elements 

### Setup - nix-shell
IMPORTANT: These need to be run in the correct nix-shell. 

In the base folder of this repository, developer-exercises, you will find
a `default.nix` file. Run the following command in your terminal:

```bash
nix-shell
```

The very first time you run this, it will take long time, somewhere between 20 and 80 minutes.
This is because it will download, install and compile everything you need. After that it will only take a second or two to run.

### Tests

```bash
cd tests
npm install
npm tests
```
