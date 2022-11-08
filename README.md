***This project is outdated. The new implementation is in `rcrisanti/moolah`.***

# moolah
a personal finance projection WASM web app built in Rust

## `backend` CRUD coverage

| endpoint    | Create (POST/PUT)  | Read (GET)         | Update (PATCH)     | Delete (DELETE)    |
| ----------- | ------------------ | ------------------ | ------------------ | ------------------ |
| login       | :white_check_mark: | :heavy_minus_sign: | :white_check_mark: | :heavy_minus_sign: |
| logout      | :white_check_mark: | :heavy_minus_sign: | :heavy_minus_sign: | :heavy_minus_sign: |
| user        | :white_check_mark: | :white_check_mark: | :x:                | :white_check_mark: |
| predictions | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| deltas      | :white_check_mark: | :x:                | :x:                | :x:                |


