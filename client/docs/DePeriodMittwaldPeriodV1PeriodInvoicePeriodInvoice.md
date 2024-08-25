# DePeriodMittwaldPeriodV1PeriodInvoicePeriodInvoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount_paid** | **f64** |  | 
**cancellation** | Option<[**models::DePeriodMittwaldPeriodV1PeriodInvoicePeriodCancellation**](de.mittwald.v1.invoice.Cancellation.md)> |  | [optional]
**cancellation_of** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The ID of the invoice that this invoice cancels. | [optional]
**currency** | **String** |  | 
**customer_id** | **String** |  | 
**date** | **String** |  | 
**groups** | [**Vec<models::DeMittwaldV1InvoiceInvoiceGroupsInner>**](de_mittwald_v1_invoice_Invoice_groups_inner.md) |  | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**invoice_number** | **String** |  | 
**invoice_type** | **String** |  | 
**payment_settings** | Option<[**models::DePeriodMittwaldPeriodV1PeriodInvoicePeriodPaymentSettings**](de.mittwald.v1.invoice.PaymentSettings.md)> |  | [optional]
**pdf_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**recipient** | [**models::DePeriodMittwaldPeriodV1PeriodInvoicePeriodRecipient**](de.mittwald.v1.invoice.Recipient.md) |  | 
**status** | **String** |  | 
**total_gross** | **f64** |  | 
**total_net** | **f64** |  | 
**vat_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


