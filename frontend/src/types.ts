export interface Quote {
    id: string
    quote: string
    tags: string
    author: {
        name: string,
        slug: string
    }
    created_by: string
    likes_count: string
    created_at: string
    updated_at: string
}

export interface User {
    id: String 
    username: String 
    token: String
}