# \ArticleApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**article_get_article**](ArticleApi.md#article_get_article) | **GET** /v2/articles/{articleId} | Get an Article.
[**article_list_articles**](ArticleApi.md#article_list_articles) | **GET** /v2/articles | List Articles.



## article_get_article

> models::DePeriodMittwaldPeriodV1PeriodArticlePeriodReadableArticle article_get_article(article_id, customer_id)
Get an Article.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**article_id** | **String** |  | [required] |
**customer_id** | Option<**String**> | not in use. |  |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodArticlePeriodReadableArticle**](de.mittwald.v1.article.ReadableArticle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_list_articles

> Vec<models::DePeriodMittwaldPeriodV1PeriodArticlePeriodReadableArticle> article_list_articles(customer_id, limit, skip, page, tags, template_names, article_ids, orderable, name)
List Articles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | not in use. |  |
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |
**page** | Option<**i32**> |  |  |
**tags** | Option<[**Vec<String>**](String.md)> |  |  |
**template_names** | Option<[**Vec<String>**](String.md)> |  |  |
**article_ids** | Option<[**Vec<String>**](String.md)> |  |  |
**orderable** | Option<[**Vec<String>**](String.md)> |  |  |
**name** | Option<**String**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodArticlePeriodReadableArticle>**](de.mittwald.v1.article.ReadableArticle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

