# \ContractApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**contract_cancel_contract_item_termination**](ContractApi.md#contract_cancel_contract_item_termination) | **DELETE** /v2/contracts/{contractId}/items/{contractItemId}/termination | Cancel the Termination for the referred ContractItem.
[**contract_cancel_contract_tariff_change**](ContractApi.md#contract_cancel_contract_tariff_change) | **DELETE** /v2/contracts/{contractId}/items/{contractItemId}/tariff-change | Cancel the TariffChange for the referred ContractItem.
[**contract_cancel_contract_termination**](ContractApi.md#contract_cancel_contract_termination) | **DELETE** /v2/contracts/{contractId}/termination | Cancel the Termination for the referred Contract.
[**contract_get_base_item_of_contract**](ContractApi.md#contract_get_base_item_of_contract) | **GET** /v2/contracts/{contractId}/base-items | Return the BaseItem of the Contract with the given ID.
[**contract_get_detail_of_contract**](ContractApi.md#contract_get_detail_of_contract) | **GET** /v2/contracts/{contractId} | Returns the Contract with the given ID.
[**contract_get_detail_of_contract_by_certificate**](ContractApi.md#contract_get_detail_of_contract_by_certificate) | **GET** /v2/certificates/{certificateId}/contract | Return the Contract for the given Certificate.
[**contract_get_detail_of_contract_by_domain**](ContractApi.md#contract_get_detail_of_contract_by_domain) | **GET** /v2/domains/{domainId}/contract | Return the Contract for the given Domain.
[**contract_get_detail_of_contract_by_project**](ContractApi.md#contract_get_detail_of_contract_by_project) | **GET** /v2/projects/{projectId}/contract | Return the Contract for the given Project.
[**contract_get_detail_of_contract_by_server**](ContractApi.md#contract_get_detail_of_contract_by_server) | **GET** /v2/servers/{serverId}/contract | Return the Contract for the given Server.
[**contract_get_detail_of_contract_item**](ContractApi.md#contract_get_detail_of_contract_item) | **GET** /v2/contracts/{contractId}/items/{contractItemId} | Get the ContractItem with the given ID.
[**contract_get_next_termination_date_for_item**](ContractApi.md#contract_get_next_termination_date_for_item) | **GET** /v2/contracts/{contractId}/items/{contractItemId}/next-termination-dates | Return the next TerminationDate for the ContractItem with the given ID.
[**contract_list_contracts**](ContractApi.md#contract_list_contracts) | **GET** /v2/customers/{customerId}/contracts | Return a list of Contracts for the given Customer.
[**contract_terminate_contract**](ContractApi.md#contract_terminate_contract) | **POST** /v2/contracts/{contractId}/termination | Schedule the Termination of a Contract.
[**contract_terminate_contract_item**](ContractApi.md#contract_terminate_contract_item) | **POST** /v2/contracts/{contractId}/items/{contractItemId}/termination | Schedule the Termination of a ContractItem.
[**deprecated_invoice_detail_of_invoice**](ContractApi.md#deprecated_invoice_detail_of_invoice) | **GET** /v2/customers/{customerId}/invoices/{invoiceId} | Get details of an Invoice.
[**invoice_detail**](ContractApi.md#invoice_detail) | **GET** /v2/invoices/{invoiceId} | Get details of an Invoice.
[**invoice_get_detail_of_invoice_settings**](ContractApi.md#invoice_get_detail_of_invoice_settings) | **GET** /v2/customers/{customerId}/invoice-settings | Get InvoiceSettings of a Customer.
[**invoice_get_file_access_token**](ContractApi.md#invoice_get_file_access_token) | **GET** /v2/customers/{customerId}/invoices/{invoiceId}/file-access-token | Request an Access Token for the Invoice file.
[**invoice_list_customer_invoices**](ContractApi.md#invoice_list_customer_invoices) | **GET** /v2/customers/{customerId}/invoices | List Invoices of a Customer.
[**invoice_update_invoice_settings**](ContractApi.md#invoice_update_invoice_settings) | **PUT** /v2/customers/{customerId}/invoice-settings | Update InvoiceSettings of a Customer.
[**order_create_order**](ContractApi.md#order_create_order) | **POST** /v2/orders | Create an Order.
[**order_create_tariff_change**](ContractApi.md#order_create_tariff_change) | **POST** /v2/tariff-changes | Create TariffChange Order.
[**order_get_order**](ContractApi.md#order_get_order) | **GET** /v2/orders/{orderId} | Get Order for Customer.
[**order_list_customer_orders**](ContractApi.md#order_list_customer_orders) | **GET** /v2/customers/{customerId}/orders | Get list of Orders of a Customer.
[**order_list_orders**](ContractApi.md#order_list_orders) | **GET** /v2/orders | Get list of Orders.
[**order_list_project_orders**](ContractApi.md#order_list_project_orders) | **GET** /v2/projects/{projectId}/orders | Get list of Orders of a Project.
[**order_preview_order**](ContractApi.md#order_preview_order) | **POST** /v2/order-previews | Preview Order.
[**order_preview_tariff_change**](ContractApi.md#order_preview_tariff_change) | **POST** /v2/tariff-change-previews | Preview TariffChange.



## contract_cancel_contract_item_termination

> models::ContractCancelContractItemTermination200Response contract_cancel_contract_item_termination(contract_id, contract_item_id)
Cancel the Termination for the referred ContractItem.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **uuid::Uuid** | The uuid of the Contract where the terminated ContractItem belongs to. | [required] |
**contract_item_id** | **uuid::Uuid** | The uuid of the ContractItem for which the Termination is to be cancelled. Only works for ContractItem for which a Termination is stored. | [required] |

### Return type

[**models::ContractCancelContractItemTermination200Response**](contract_cancel_contract_item_termination_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_cancel_contract_tariff_change

> models::ContractCancelContractItemTermination200Response contract_cancel_contract_tariff_change(contract_id, contract_item_id)
Cancel the TariffChange for the referred ContractItem.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **uuid::Uuid** |  | [required] |
**contract_item_id** | **uuid::Uuid** | The uuid of the active ContractItem (not the one that would replace the old one after the TariffChange) for which the TariffChange is to be cancelled. Only works for ContractItems for which a TariffChange is stored.  | [required] |

### Return type

[**models::ContractCancelContractItemTermination200Response**](contract_cancel_contract_item_termination_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_cancel_contract_termination

> models::ContractCancelContractTermination200Response contract_cancel_contract_termination(contract_id)
Cancel the Termination for the referred Contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **uuid::Uuid** | The uuid of the Contract for which the Termination is to be cancelled. Only works for Contracts for which a Termination is stored.  | [required] |

### Return type

[**models::ContractCancelContractTermination200Response**](contract_cancel_contract_termination_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_get_base_item_of_contract

> models::DePeriodMittwaldPeriodV1PeriodContractPeriodContractItem contract_get_base_item_of_contract(contract_id)
Return the BaseItem of the Contract with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **uuid::Uuid** | The uuid of the Contract from which the BaseItem is to be issued. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodContractPeriodContractItem**](de.mittwald.v1.contract.ContractItem.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_get_detail_of_contract

> models::DePeriodMittwaldPeriodV1PeriodContractPeriodContract contract_get_detail_of_contract(contract_id)
Returns the Contract with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **uuid::Uuid** | The uuid of the Contract to be returned. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodContractPeriodContract**](de.mittwald.v1.contract.Contract.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_get_detail_of_contract_by_certificate

> models::DePeriodMittwaldPeriodV1PeriodContractPeriodContract contract_get_detail_of_contract_by_certificate(certificate_id)
Return the Contract for the given Certificate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodContractPeriodContract**](de.mittwald.v1.contract.Contract.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_get_detail_of_contract_by_domain

> models::DePeriodMittwaldPeriodV1PeriodContractPeriodContract contract_get_detail_of_contract_by_domain(domain_id)
Return the Contract for the given Domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodContractPeriodContract**](de.mittwald.v1.contract.Contract.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_get_detail_of_contract_by_project

> models::DePeriodMittwaldPeriodV1PeriodContractPeriodContract contract_get_detail_of_contract_by_project(project_id)
Return the Contract for the given Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodContractPeriodContract**](de.mittwald.v1.contract.Contract.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_get_detail_of_contract_by_server

> models::DePeriodMittwaldPeriodV1PeriodContractPeriodContract contract_get_detail_of_contract_by_server(server_id)
Return the Contract for the given Server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodContractPeriodContract**](de.mittwald.v1.contract.Contract.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_get_detail_of_contract_item

> models::DePeriodMittwaldPeriodV1PeriodContractPeriodContractItem contract_get_detail_of_contract_item(contract_id, contract_item_id)
Get the ContractItem with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **uuid::Uuid** | The uuid of the Contract where the desired ContractItem belongs to. | [required] |
**contract_item_id** | **uuid::Uuid** | The uuid of the ContractItem to be returned. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodContractPeriodContractItem**](de.mittwald.v1.contract.ContractItem.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_get_next_termination_date_for_item

> models::ContractGetNextTerminationDateForItem200Response contract_get_next_termination_date_for_item(contract_id, contract_item_id)
Return the next TerminationDate for the ContractItem with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **uuid::Uuid** | The uuid of the Contract where the desired ContractItem belongs to. | [required] |
**contract_item_id** | **uuid::Uuid** | The uuid of the ContractItem whose next TerminationDate is to be displayed. | [required] |

### Return type

[**models::ContractGetNextTerminationDateForItem200Response**](contract_get_next_termination_date_for_item_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_list_contracts

> Vec<models::DePeriodMittwaldPeriodV1PeriodContractPeriodContract> contract_list_contracts(customer_id, limit, skip, page)
Return a list of Contracts for the given Customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **uuid::Uuid** | The uuid of the Customer from whom all Contracts are to be returned. | [required] |
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |
**page** | Option<**i32**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodContractPeriodContract>**](de.mittwald.v1.contract.Contract.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_terminate_contract

> models::ContractTerminateContract201Response contract_terminate_contract(contract_id, contract_terminate_contract_request)
Schedule the Termination of a Contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **uuid::Uuid** | The uuid of the Contract to be terminated. | [required] |
**contract_terminate_contract_request** | [**ContractTerminateContractRequest**](ContractTerminateContractRequest.md) |  | [required] |

### Return type

[**models::ContractTerminateContract201Response**](contract_terminate_contract_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_terminate_contract_item

> models::ContractTerminateContractItem201Response contract_terminate_contract_item(contract_id, contract_item_id, contract_terminate_contract_item_request)
Schedule the Termination of a ContractItem.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **uuid::Uuid** |  | [required] |
**contract_item_id** | **uuid::Uuid** |  | [required] |
**contract_terminate_contract_item_request** | [**ContractTerminateContractItemRequest**](ContractTerminateContractItemRequest.md) |  | [required] |

### Return type

[**models::ContractTerminateContractItem201Response**](contract_terminate_contract_item_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deprecated_invoice_detail_of_invoice

> models::DePeriodMittwaldPeriodV1PeriodInvoicePeriodInvoice deprecated_invoice_detail_of_invoice(customer_id, invoice_id)
Get details of an Invoice.

This route is deprecated. Use /v2/invoices/{invoiceId} instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**invoice_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodInvoicePeriodInvoice**](de.mittwald.v1.invoice.Invoice.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoice_detail

> models::DePeriodMittwaldPeriodV1PeriodInvoicePeriodInvoice invoice_detail(invoice_id)
Get details of an Invoice.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodInvoicePeriodInvoice**](de.mittwald.v1.invoice.Invoice.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoice_get_detail_of_invoice_settings

> models::DePeriodMittwaldPeriodV1PeriodInvoicePeriodInvoiceSettings invoice_get_detail_of_invoice_settings(customer_id)
Get InvoiceSettings of a Customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodInvoicePeriodInvoiceSettings**](de.mittwald.v1.invoice.InvoiceSettings.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoice_get_file_access_token

> models::ConversationGetFileAccessToken200Response invoice_get_file_access_token(customer_id, invoice_id)
Request an Access Token for the Invoice file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**invoice_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ConversationGetFileAccessToken200Response**](conversation_get_file_access_token_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoice_list_customer_invoices

> Vec<models::DePeriodMittwaldPeriodV1PeriodInvoicePeriodInvoice> invoice_list_customer_invoices(customer_id, invoice_types, limit, skip, page)
List Invoices of a Customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**invoice_types** | Option<[**Vec<String>**](String.md)> |  |  |
**limit** | Option<**i32**> | The max count of resources to return in the list response. |  |[default to 1000]
**skip** | Option<**i32**> | Number of items to skip. Should be multiple of `limit`. |  |[default to 0]
**page** | Option<**i32**> | Page to display. `skip` and `page` end up doing the same. If both `page` and `skip` are set, skip is used. |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodInvoicePeriodInvoice>**](de.mittwald.v1.invoice.Invoice.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoice_update_invoice_settings

> models::DePeriodMittwaldPeriodV1PeriodInvoicePeriodInvoiceSettings invoice_update_invoice_settings(customer_id, invoice_update_invoice_settings_request)
Update InvoiceSettings of a Customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**invoice_update_invoice_settings_request** | [**InvoiceUpdateInvoiceSettingsRequest**](InvoiceUpdateInvoiceSettingsRequest.md) |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodInvoicePeriodInvoiceSettings**](de.mittwald.v1.invoice.InvoiceSettings.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_create_order

> models::OrderCreateOrder201Response order_create_order(order_create_order_request)
Create an Order.



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_create_order_request** | [**OrderCreateOrderRequest**](OrderCreateOrderRequest.md) | The Order to create. | [required] |

### Return type

[**models::OrderCreateOrder201Response**](order_create_order_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_create_tariff_change

> models::OrderCreateOrder201Response order_create_tariff_change(order_create_tariff_change_request)
Create TariffChange Order.



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_create_tariff_change_request** | [**OrderCreateTariffChangeRequest**](OrderCreateTariffChangeRequest.md) | The Order to create. | [required] |

### Return type

[**models::OrderCreateOrder201Response**](order_create_order_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_get_order

> models::DePeriodMittwaldPeriodV1PeriodOrderPeriodCustomerOrder order_get_order(order_id)
Get Order for Customer.

Get details of a single Order, User must have access to the Order/Customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodOrderPeriodCustomerOrder**](de.mittwald.v1.order.CustomerOrder.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_list_customer_orders

> Vec<models::DePeriodMittwaldPeriodV1PeriodOrderPeriodCustomerOrder> order_list_customer_orders(customer_id, limit, skip, page, includes_status, excludes_status, template_names)
Get list of Orders of a Customer.

The list of Orders of a Customer the User has access to, can be filtered by orderStatus, articleTemplate and count.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |
**page** | Option<**i32**> |  |  |
**includes_status** | Option<[**Vec<models::DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderStatus>**](models::DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderStatus.md)> |  |  |
**excludes_status** | Option<[**Vec<models::DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderStatus>**](models::DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderStatus.md)> |  |  |
**template_names** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodOrderPeriodCustomerOrder>**](de.mittwald.v1.order.CustomerOrder.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_list_orders

> Vec<models::DePeriodMittwaldPeriodV1PeriodOrderPeriodCustomerOrder> order_list_orders(limit, skip, page, includes_status, excludes_status, template_names)
Get list of Orders.

The list of Orders the User has access to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |
**page** | Option<**i32**> |  |  |
**includes_status** | Option<[**Vec<models::DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderStatus>**](models::DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderStatus.md)> |  |  |
**excludes_status** | Option<[**Vec<models::DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderStatus>**](models::DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderStatus.md)> |  |  |
**template_names** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodOrderPeriodCustomerOrder>**](de.mittwald.v1.order.CustomerOrder.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_list_project_orders

> Vec<models::DePeriodMittwaldPeriodV1PeriodOrderPeriodCustomerOrder> order_list_project_orders(project_id, limit, skip, page, includes_status, excludes_status, template_names)
Get list of Orders of a Project.

The list of Order of a Project the User has access to, can be filtered by orderStatus, articleTemplate and count.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |
**page** | Option<**i32**> |  |  |
**includes_status** | Option<[**Vec<models::DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderStatus>**](models::DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderStatus.md)> |  |  |
**excludes_status** | Option<[**Vec<models::DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderStatus>**](models::DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderStatus.md)> |  |  |
**template_names** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodOrderPeriodCustomerOrder>**](de.mittwald.v1.order.CustomerOrder.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_preview_order

> models::OrderPreviewOrder200Response order_preview_order(order_preview_order_request)
Preview Order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_preview_order_request** | [**OrderPreviewOrderRequest**](OrderPreviewOrderRequest.md) | Your order information | [required] |

### Return type

[**models::OrderPreviewOrder200Response**](order_preview_order_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_preview_tariff_change

> models::OrderPreviewTariffChange200Response order_preview_tariff_change(order_create_tariff_change_request)
Preview TariffChange.



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_create_tariff_change_request** | [**OrderCreateTariffChangeRequest**](OrderCreateTariffChangeRequest.md) | Your order information | [required] |

### Return type

[**models::OrderPreviewTariffChange200Response**](order_preview_tariff_change_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

