<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetSku</name>
   <tag></tag>
   <elementGuidId>96329e44-5c81-45c1-bf1e-eb0b4168e5ab</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${product-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setSku\&quot;,\n            \&quot;variantId\&quot; : 1,\n            \&quot;sku\&quot; : \&quot;SKU\&quot;\n          }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>fdebc4c7-256d-4624-a82a-d03a3b41b746</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>3c76e44e-b4d6-4ffe-831d-d92bfc5ec8cf</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/products/${product-id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>10c6125b-472e-4494-a0d6-2b4bdcb74e92</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>6df00db6-6207-451e-950e-ebc5eb11c1bb</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.product-id</defaultValue>
      <description></description>
      <id>d0fde603-7523-4282-a96f-d1516cdbd8e1</id>
      <masked>false</masked>
      <name>product-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.product-version</defaultValue>
      <description></description>
      <id>128a439e-fb39-4118-99e3-12ac805c7dff</id>
      <masked>false</masked>
      <name>product-version</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
