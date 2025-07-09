# JSON-LD Structured Data Files

This directory contains JSON-LD structured data files that are used for SEO and search engine optimization.

## Files

### `website.json`
Contains website-level structured data including:
- Website name and URL
- Search functionality metadata
- General site information

### `person.json`
Contains personal/professional structured data including:
- Personal information (name, job title)
- Skills and knowledge areas
- Social media profiles
- Contact information
- Location data

## Usage

These files are automatically loaded by the Rust application and included in the HTML `<head>` section of every page. The structured data helps search engines understand your website content and can improve search result appearance.

## Editing

To modify the structured data:

1. Edit the JSON files directly in this directory
2. Restart the application for changes to take effect
3. Validate your JSON-LD using Google's Structured Data Testing Tool

## API Access

The JSON-LD data is also available via HTTP endpoints:
- `GET /api/json-ld/website` - Returns website.json
- `GET /api/json-ld/person` - Returns person.json

## Important Notes

- Keep the `@context` and `@type` fields as they define the schema
- Update URLs to match your actual domain
- Remove or modify the `potentialAction` in website.json if you don't have site search
- Ensure email addresses and social media URLs are correct
- The files are served as static assets, so they're also accessible at `/static/data/`

## Schema.org References

- [WebSite Schema](https://schema.org/WebSite)
- [Person Schema](https://schema.org/Person)
- [JSON-LD Documentation](https://json-ld.org/)