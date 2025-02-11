<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddCustomLineItem</name>
   <tag></tag>
   <elementGuidId>18fda890-9d2b-4673-9dfa-362c3da38082</elementGuidId>
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
      <webElementGuid>4330f9f5-8f3a-4733-846b-860f39e413f3</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>df78c722-b495-46f3-8a93-feaf6cdafeee</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/carts/${cart-id}</restUrl>
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
      <id>cdd7e036-a026-4623-90bf-f85fc2a74b04</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>1ada02f6-5c2d-4d73-8f7a-001fed7547c3</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-id</defaultValue>
      <description></description>
      <id>96218a00-e52d-44ac-9639-a45b6aa3ac56</id>
      <masked>false</masked>
      <name>cart-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-version</defaultValue>
      <description></description>
      <id>878906e6-6520-43c4-9b62-3defcfb4d008</id>
      <masked>false</masked>
      <name>cart-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tax-category-id</defaultValue>
      <description></description>
      <id>5f057ec5-db4b-43d3-bf4e-593a098967ae</id>
      <masked>false</masked>
      <name>tax-category-id</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
