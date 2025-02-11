<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddCustomLineItem</name>
   <tag></tag>
   <elementGuidId>b0eb4bef-1246-4969-bb85-4d9fabaa3597</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${cart-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;addCustomLineItem\&quot;,\n            \&quot;name\&quot; : {\n              \&quot;en\&quot; : \&quot;Name EN\&quot;,\n              \&quot;de\&quot; : \&quot;Name DE\&quot;\n            },\n            \&quot;quantity\&quot; : 1,\n            \&quot;money\&quot; : {\n              \&quot;currencyCode\&quot; : \&quot;EUR\&quot;,\n              \&quot;centAmount\&quot; : 4200\n            },\n            \&quot;slug\&quot; : \&quot;mySlug\&quot;,\n            \&quot;taxCategory\&quot; : {\n              \&quot;typeId\&quot; : \&quot;tax-category\&quot;,\n              \&quot;id\&quot; : \&quot;${tax-category-id}\&quot;\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>f3d86252-900a-46a4-a910-49abacc947be</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>6f66760a-8d2f-4496-8b27-65487899d035</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/in-store/key=${store-key}/carts/${cart-id}</restUrl>
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
      <id>da8bd9ba-48c3-4014-9e85-ddc37b981a1b</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>427ee62a-a0a9-4058-b5be-485bae3678c4</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.store-key</defaultValue>
      <description></description>
      <id>b60e7af4-a9d5-4652-93c5-a5096f2d66b6</id>
      <masked>false</masked>
      <name>store-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-id</defaultValue>
      <description></description>
      <id>6e6684d4-6d16-42c1-9ab0-604942ba76cf</id>
      <masked>false</masked>
      <name>cart-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-version</defaultValue>
      <description></description>
      <id>aa9ab3b3-a2f3-4512-b0ea-96abad080653</id>
      <masked>false</masked>
      <name>cart-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tax-category-id</defaultValue>
      <description></description>
      <id>bb1394ee-132b-402a-9a63-c3e4b75d94dd</id>
      <masked>false</masked>
      <name>tax-category-id</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
