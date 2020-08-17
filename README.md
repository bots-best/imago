# Imago

Imago is a service that lets you upload images and then load them again
in different formats and sizes.

## Documentation

Imago exposes a Http Json API to talk with clients.

### Uploading images

``` 
POST /upload
```

**Request**

```json
{
  "url": "url to some image",
  "sizes": [
    { "width": 740, "height": 900 },
    { "width": 100, "height": 100 },
    "16x16"
  ]
}
```

**Response**

```json
{
    "id": "3528f40e-2e14-415c-bad3-312dceb27c1b",
}
```

### Retrieving images

```
GET /get/<id>
```

**Url Parameters**

- `format`: The image format (e.g. `jpeg`, `jpg`, `png`)
- `size`: The width and height of the image (e.g. `1920x1080`)

(Currently it's just a sketch and can be changed in the future.)
