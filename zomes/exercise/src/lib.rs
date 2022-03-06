use hdk::prelude::*;

entry_defs![Post::entry_def(), Anchor::entry_def()];

#[hdk_entry(id = "post")]
pub struct Post(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalInput {
    content: String,
}

#[hdk_extern]
pub fn create_post(input: SomeExternalInput) -> ExternResult<HeaderHash> {
    let post_anchor = anchor(
    String::from("ALL_POSTS"),
    String::from("ALL_POSTS"))?;

    let post = Post(input.content);
    create_entry(&post)?;
    let hash_entry = hash_entry(&post)?;

    let link_hash = create_link(post_anchor,hash_entry, LinkTag::new(String::from("all_posts")))?;
    Ok(link_hash)
}

#[hdk_extern]
pub fn get_all_posts(_: ()) -> ExternResult<Vec<Post>> {
    let post_anchor = anchor(
        String::from("ALL_POSTS"),
        String::from("ALL_POSTS"))?;

    let anchor_links = get_links(post_anchor, None)?;


    let mut post_list:Vec<Post> = vec![];

    for posts in anchor_links {
        let element: Element = get(posts.target, GetOptions::default())?
            .ok_or(WasmError::Guest(String::from("unable to retrieve elements")))?;

        let post = element.entry() // access the entry field
            .to_app_option()?   // deserialise and convert to option
            .ok_or(WasmError::Guest(String::from("no content in Post")))?;  // handle Error 


        post_list.push(Post::from(post));
    }

    Ok(post_list)
}